#include <unicode/uniset.h>
#include <stdio.h>
#include <chrono>

using namespace std;

#define NUM(a) (sizeof(a) / sizeof(*a))

static void show(void);

void show(void)
{
    auto bestSample = new icu::UnicodeSet('A', 'F');

    auto worstSample = new icu::UnicodeSet();
    for (u_int32_t i = 0; i < 1114111 + 1; i+=2) {
        worstSample->add(i);
    }

    auto total = 0;
    auto start = chrono::steady_clock::now();

    for (u_int32_t i = 0; i < 5; i++) {
        auto c = bestSample->charAt(i);
        if (bestSample->contains(c)) {
            total += 1;
        }
    }

    for (u_int32_t i = 0; i < 100; i++) {
        auto c = worstSample->charAt(i);
        if (worstSample->contains(c)) {
            total += 1;
        }
    }

    for (u_int32_t i = 0; i < 5; i++) {
        auto c = bestSample->charAt(i);
        if (bestSample->contains('A', c)) {
            total += 1;
        }
    }

    for (u_int32_t i = 0; i < 100; i++) {
        auto c = worstSample->charAt(i);
        if (worstSample->contains(0, c)) {
            total += 1;
        }
    }

    auto end = chrono::steady_clock::now();
    auto diff = end - start;
    auto measured_ns = chrono::duration_cast<chrono::nanoseconds>(diff).count();
    printf("UnicodeSet test: %lld ns, total: %lld \n", measured_ns, total);
}


int main() {
  show();
  return 0;
}
