CXX := c++

# System ICU
# PKG := `pkg-config --libs --cflags icu-uc icu-io icu-i18n jsoncpp`

# Custom ICU
ICU_PATH:= /home/zbraniecki/projects/icu/icu4c/source
PKG := `pkg-config --libs --cflags jsoncpp` -I$(ICU_PATH)/i18n -I$(ICU_PATH)/common -L$(ICU_PATH)/lib -licui18n -licuuc -licudata

all: locale pluralrules date uniset line_break

locale: locale.cpp
	$(CXX) -std=c++20 -O3 -o locale locale.cpp $(PKG)
date: date.cpp
	$(CXX) -std=c++20 -O3 -o date date.cpp $(PKG)
pluralrules: pluralrules.cpp
	$(CXX) -std=c++20 -O3 -o pluralrules pluralrules.cpp $(PKG)
uniset: uniset.cpp
	$(CXX) -std=c++20 -O3 -o uniset uniset.cpp $(PKG)
line_break: line_break.cpp
	$(CXX) -std=c++20 -O3 -o line_break line_break.cpp $(PKG)

clean:
	rm -f locale pluralrules date uniset line_break
