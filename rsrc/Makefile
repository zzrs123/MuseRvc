CC = cargo build
# CFLAGS = --crate-type=bin
# TARGET = target/debug/muservc

all: $(TARGET)

$(TARGET): src/main.rs
	$(CC) 

.PHONY: test
test: $(TARGET)
	$(CC) 
	bash test.sh

.PHONY: clean
clean:
	rm -rf target/