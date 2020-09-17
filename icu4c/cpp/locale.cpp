#include <unicode/locid.h>
#include <unicode/utypes.h>
#include <unicode/bytestream.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>
#include <json/value.h>
#include "json/json.h"
#include <fstream>

using namespace std;

#define NUM(a) (sizeof(a) / sizeof(*a))

static void show(void);

void show(void)
{
	Json::Value root;   // will contains the root value after parsing.
	Json::Reader reader;
	std::ifstream test("../../harness/data/locale/langids.json", std::ifstream::binary);
	bool parsingSuccessful = reader.parse( test, root, false );


	std::vector<std::string> langids;
	vector<icu::Locale> locales;

	for (Json::Value::ArrayIndex i = 0; i != root.size(); i++) {
		langids.push_back(root[i]["input"].asString());
	}

	printf("[\n");

	{
		// Construct
		auto start = chrono::steady_clock::now();

		for (auto& langid : langids) {
			icu::Locale locale = icu::Locale(langid.c_str());
			locales.push_back(locale);
		}

		auto end = chrono::steady_clock::now();
		auto diff = end - start;
		auto measured_ns = chrono::duration_cast<chrono::nanoseconds>(diff).count();
		printf("{\"id\":\"langid/construct\",\"result\":%ld,\"unit\":\"ns\"},\n", measured_ns);

	}


	{
		// Compare
		icu::Locale referenceLocale = icu::Locale("en-US");

		vector<icu::Locale> filtered;

		auto start = chrono::steady_clock::now();

		for (auto& loc : locales) {
			if (loc == referenceLocale) {
				filtered.push_back(loc);
			}
		}

		auto end = chrono::steady_clock::now();
		auto diff = end - start;
		auto measured_ns = chrono::duration_cast<chrono::nanoseconds>(diff).count();
		printf("{\"id\":\"langid/filter\",\"result\":%ld,\"unit\":\"ns\"},\n", measured_ns);
	}

	{
		// ToString

		std::vector<std::string> results;

		UErrorCode status = U_ZERO_ERROR;

		auto start = chrono::steady_clock::now();

		for (auto& loc : locales) {
			string dest = "";
			icu::StringByteSink<std::string> sink(&dest);
			loc.toLanguageTag(sink, status);
			results.push_back(dest);
		}

		auto end = chrono::steady_clock::now();
		auto diff = end - start;
		auto measured_ns = chrono::duration_cast<chrono::nanoseconds>(diff).count();
		printf("{\"id\":\"langid/serialize\",\"result\":%ld,\"unit\":\"ns\"},\n", measured_ns);
	}

	{
		// Canonicalize

		std::vector<std::string> results;

		UErrorCode status = U_ZERO_ERROR;

		auto start = chrono::steady_clock::now();

		for (auto& langid : langids) {
			icu::Locale locale = icu::Locale(langid.c_str());
			string dest = "";
			icu::StringByteSink<std::string> sink(&dest);
			locale.toLanguageTag(sink, status);
			results.push_back(dest);
		}

		auto end = chrono::steady_clock::now();
		auto diff = end - start;
		auto measured_ns = chrono::duration_cast<chrono::nanoseconds>(diff).count();
		printf("{\"id\":\"langid/canonicalize\",\"result\":%ld,\"unit\":\"ns\"},\n", measured_ns);
	}

	{
		// Size

		// XXX: We suspect that this does not measure heap allocation, leaving `variants` not accounted for.
		// Since there are only 3 locales with variants in the sample, we hope the results
		// remain meaningful, but it would be nice to improve the measuring.

		unsigned int size = sizeof(decltype(locales.back())) * locales.capacity();
		printf("{\"id\":\"langid/memory\",\"result\":%d,\"unit\":\"b\"}\n", size);
	}
	printf("]\n");

}


int main() {
	show();
	return 0;
}
