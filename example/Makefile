CC = cc
RM = rm -f
CPPFLAGS = -g -I ../include
LDFLAGS = -g
LDLIBS = -L ../target/release -lhtml2html_lib

SRCS = html2html.c
OBJS = $(subst .c,.o,$(SRCS))

all: html2html

html2html: $(OBJS)
	$(CC) $(LDFLAGS) -o html2html $(OBJS) $(LDLIBS)

clean:
	$(RM) $(OBJS)

dist-clean: clean
	$(RM) html2html
