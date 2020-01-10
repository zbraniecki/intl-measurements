#include <unicode/locid.h>
#include <unicode/utypes.h>
#include <unicode/datefmt.h>
#include <unicode/bytestream.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>

using namespace std;

static void show(void);

void show(void)
{
  {
    UDate dates[] = {
      1000000000000.0,
      1500000000000.0,
      1600000000000.0,
      1610000000000.0,
      1620000000000.0,
      1630000000000.0,
      1640000000000.0,
      1650000000000.0,
      1660000000000.0,
      2000000000000.0,
    };
    const char* locales[] = {
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
      "pl",
    };
    icu_65::DateFormat::EStyle dateStyles[] = {
      icu_65::DateFormat::EStyle::FULL,
      icu_65::DateFormat::EStyle::LONG,
      icu_65::DateFormat::EStyle::MEDIUM,
      icu_65::DateFormat::EStyle::SHORT,
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::FULL,
      icu_65::DateFormat::EStyle::LONG,
      icu_65::DateFormat::EStyle::MEDIUM,
      icu_65::DateFormat::EStyle::SHORT,
    };
    icu_65::DateFormat::EStyle timeStyles[] = {
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::NONE,
      icu_65::DateFormat::EStyle::FULL,
      icu_65::DateFormat::EStyle::LONG,
      icu_65::DateFormat::EStyle::MEDIUM,
      icu_65::DateFormat::EStyle::SHORT,
      icu_65::DateFormat::EStyle::FULL,
      icu_65::DateFormat::EStyle::LONG,
      icu_65::DateFormat::EStyle::MEDIUM,
      icu_65::DateFormat::EStyle::SHORT,
    };
    icu_65::Locale pl = icu_65::Locale("pl");
    auto start = chrono::steady_clock::now();

    for (auto i = 0; i < 12; i++) {
      auto dtf = icu_65::DateFormat::createDateTimeInstance(dateStyles[i], timeStyles[i], locales[i]);
      for (auto& dat : dates) {
        icu_65::UnicodeString myString;
        dtf->format( dat, myString );

        //std::string utf8;
        //myString.toUTF8String(utf8);
        //printf("date: %s\n", utf8.c_str());
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
