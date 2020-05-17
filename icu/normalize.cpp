#include <unicode/normalizer2.h>
#include <chrono>
#include <fstream>

using namespace std;

string readFile(const char* path)
{
  std::ifstream t(path);
  std::string str((std::istreambuf_iterator<char>(t)),
                   std::istreambuf_iterator<char>());
  return str;
}

void show(void)
{

  UErrorCode status = U_ZERO_ERROR;

  // Note: this is not great.
  // The file is read as UTF-8 and not UTF-16.
  string en_sample = readFile("../data/normalization/en.txt");

  {
    auto instance = icu::Normalizer2::getNFCInstance(status);

    auto start = chrono::steady_clock::now();

    auto result = instance->normalize(en_sample.c_str(), status);

    auto end = chrono::steady_clock::now();
    auto diff = end - start;
    auto measured_us = chrono::duration_cast<chrono::microseconds>(diff).count();
    printf("Normalization to NFC of English sample. time: %d us", measured_us);

  }
}


int main() {
  show();
  return 0;
}
