#include <unicode/locid.h>
#include <unicode/utypes.h>
#include <unicode/datefmt.h>
#include <unicode/bytestream.h>
#include <unicode/plurrule.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>

using namespace std;

static void show(void);

void show(void)
{
  {
    const char* langids[] = {
      "uk",
      "de",
      "sk",
      "ar",
      "fr",
      "it",
      "en",
      "cs",
      "es",
      "zh"
    };
    const int32_t samples[] = {
      1,
      2,
      3,
      4,
      5,
      25,
      134,
      910293019,
      12,
      1412,
      -12,
      15,
      2931,
      31231,
      3123,
      13231,
      91,
      0,
      231,
      -2,
      -45,
      33,
      728,
      2,
      291,
      24,
      479,
      291,
      778,
      919,
      93
    };
    UErrorCode ec = U_ZERO_ERROR;

    vector<icu_65::Locale> locales;
    for (auto& langid : langids) {
      auto loc = icu_65::Locale(langid);
      locales.push_back(loc);
    }

    auto start = chrono::steady_clock::now();

    for (auto& loc : locales) {
      auto pr = icu_65::PluralRules::forLocale(loc, ec);
      for (auto& sample : samples) {
        auto result = pr->select(sample);

        // std::string utf8;
        // result.toUTF8String(utf8);
        // printf("pr: %s\n", utf8.c_str());
      }
    }

    auto end = chrono::steady_clock::now();
    auto diff = end - start;
    auto measured_us = chrono::duration_cast<chrono::microseconds>(diff).count();
    printf("time: %d us\n", measured_us);

  }
}


int main() {
  show();
  return 0;
}
