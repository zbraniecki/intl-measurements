#include <unicode/locid.h>
#include <unicode/utypes.h>
#include <unicode/datefmt.h>
#include <unicode/bytestream.h>
#include <unicode/plurrule.h>
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
	std::ifstream test("../../harness/data/pluralrules/plurals.json", std::ifstream::binary);
	bool parsingSuccessful = reader.parse( test, root, false );

	std::vector<std::string> langids;
	std::vector<int32_t> values;

	for (Json::Value::ArrayIndex i = 0; i != root["langids"].size(); i++) {
		langids.push_back(root["langids"][i].asString());
	}

	for (Json::Value::ArrayIndex i = 0; i != root["values"]["isize"].size(); i++) {
		values.push_back(root["values"]["isize"][i].asInt());
	}


	UErrorCode ec = U_ZERO_ERROR;

	vector<icu::Locale> locales;
	for (auto& langid : langids) {
		auto loc = icu::Locale(langid.c_str());
		locales.push_back(loc);
	}

	vector<icu::PluralRules*> prs;
	for (auto& loc : locales) {
		icu::PluralRules* pr = icu::PluralRules::forLocale(loc, ec);
		prs.push_back(pr);
	}

	auto start = chrono::steady_clock::now();

	for (auto& pr : prs) {
		for (int32_t value : values) {
			auto result = pr->select(value);

			// std::string utf8;
			// result.toUTF8String(utf8);
			// printf("pr: %s\n", utf8.c_str());
		}
	}

	auto end = chrono::steady_clock::now();
	auto diff = end - start;
	auto measured_ns = chrono::duration_cast<chrono::nanoseconds>(diff).count();
	printf("{\"id\":\"plurals/select\",\"result\":%ld,\"unit\":\"ns\"}\n", measured_ns);
}


int main() {
  show();
  return 0;
}
