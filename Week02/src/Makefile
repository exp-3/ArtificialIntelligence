.PHONY: default run clean
default:
	g++ -std=c++14 Map.cpp Canvas.cpp Graph.cpp main.cpp -I. -lglfw -lGL `pkg-config --libs --cflags opencv` -o TMS

run:
	./TMS

clean:
	rm -f TMS
