CXX := c++

all: locale date pluralrules

locale: locale.cpp
	$(CXX) -O3 -o locale locale.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`
date: date.cpp
	$(CXX) -O3 -o date date.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`
pluralrules: pluralrules.cpp
	$(CXX) -O3 -o pluralrules pluralrules.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`
normalize: normalize.cpp
	$(CXX) -O3 -o normalize normalize.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`

clean:
	rm -f date locale pluralrules normalize
