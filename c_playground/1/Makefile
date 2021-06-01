################################################################################

CC=clang # define the compiler to use
CFLAGS=-Wall -g
TARGET=learning # define the name of the executable
SOURCES=main.c lib.c

################################################################################

# define list of objects
OBJSC=$(SOURCES:.c=.o)
OBJS=$(OBJSC:.cpp=.o)

# the target is obtained linking all .o files
all: $(SOURCES) $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -o $(TARGET)

purge: clean
	rm -f $(TARGET)

clean:
	rm -f *.o

execute: 
	./learning

run: $(TARGET) execute purge

###############################################################################
#################################################################################
