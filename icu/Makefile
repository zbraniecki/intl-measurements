CXX := c++

all: locale date

locale: locale.cpp
	$(CXX) -O3 -std=c++11 -o locale locale.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`

date: date.cpp
	$(CXX) -O3 -std=c++11 -o date date.cpp `pkg-config --libs --cflags icu-uc icu-io icu-i18n`

clean:
	rm locale
	rm date
