locale: locale.cpp
	c++ -O3 -o locale locale.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`
date: date.cpp
	c++ -O3 -o date date.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`
pluralrules: pluralrules.cpp
	c++ -O3 -o pluralrules pluralrules.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`

clean:
	rm -rf date locale pluralrules
