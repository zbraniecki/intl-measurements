#include <unicode/locid.h>
#include <unicode/rbbi.h>
#include <unicode/brkiter.h>

#include <fstream>
#include <iostream>
#include <sstream>

#include <unicode/utypes.h>
#include <unicode/bytestream.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>

using namespace std;

void test(const char* testDataPath, const char* locale) {
  // Open test file.
  ifstream file(testDataPath);
  stringstream sstr;
  sstr << file.rdbuf();
  std::string strUtf8 = sstr.str();
  const auto strUtf16 = icu::UnicodeString::fromUTF8(strUtf8);

  // Initialize line breaker.
  auto start = chrono::steady_clock::now();
  UErrorCode status = U_ZERO_ERROR;
  icu::BreakIterator* iter = icu::BreakIterator::createLineInstance(
      icu::Locale(locale), status);
  iter->setText(strUtf16);
  auto end = chrono::steady_clock::now();
  cout << "Initialize line breaker: "
       << chrono::duration_cast<chrono::microseconds>(end - start).count()
       << "ms" << endl;

  // Iterate line break opportunity.
  start = chrono::steady_clock::now();
  int32_t nextPos = UBRK_DONE;
  vector<int32_t> breaks;
  while((nextPos = iter->next()) != UBRK_DONE) {
    breaks.push_back(nextPos);
  }
  end = chrono::steady_clock::now();

  // cout << "Line break opportunities:" << endl;
  // for (const auto& pos: breaks) {
  //   cout << pos << " ";
  // }
  // cout << endl;
  cout << "Iterate line break opportunities: "
       << chrono::duration_cast<chrono::microseconds>(end - start).count()
       << "ms" << endl;
}

int main() {
  cout << "Testing zhuangzi-en.txt" << endl;
  test("../../data/zhuangzi-en.txt", "en");

  cout << "Testing zhuangzi-zh.txt" << endl;
  test("../../data/zhuangzi-zh.txt", "zh");

  cout << endl;
  return 0;
}