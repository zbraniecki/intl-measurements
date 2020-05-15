#include <unicode/normalizer2.h>
#include <chrono>
#include <fstream>

using namespace std;

wstring readFile(const char* filename)
{
    std::wifstream wif(filename);
    wif.imbue(std::locale(std::locale::empty(), new std::codecvt_utf8<wchar_t>));
    std::wstringstream wss;
    wss << wif.rdbuf();
    return wss.str();
}

void show(void)
{

  // UErrorCode status = U_ZERO_ERROR;
  //
  // string en_sample = get_file_string();
  //
  // {
  //   auto instance = icu::Normalizer2::getNFCInstance(status);
  //
  //   auto start = chrono::steady_clock::now();
  //
  //   auto result = instance->normalize(en_sample.c_str(), status);
  //
  //   auto end = chrono::steady_clock::now();
  //   auto diff = end - start;
  //   auto measured_us = chrono::duration_cast<chrono::microseconds>(diff).count();
  //   printf("Normalization to NFC of English sample. time: %d us", measured_us);
  //
  // }
}


int main() {
  show();
  return 0;
}
