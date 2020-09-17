CXX := c++

all: locale pluralrules date uniset

locale: locale.cpp
	$(CXX) -std=c++17 -O3 -o locale locale.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n jsoncpp`
date: date.cpp
	$(CXX) -std=c++17 -O3 -o date date.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`
pluralrules: pluralrules.cpp
	$(CXX) -std=c++17 -O3 -o pluralrules pluralrules.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n jsoncpp`
uniset: uniset.cpp
	$(CXX) -std=c++17 -O3 -o uniset uniset.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`

clean:
	rm -f locale pluralrules date uniset
