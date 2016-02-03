obj/ub3rparse.res:
	windres src/ub3rparse.rc -O coff -o obj/ub3rparse.res

lib/libhashing.a:
	gcc -c src/hashing.c -o obj/hashing.o
	ar rcs lib/libhashing.a obj/hashing.o

lib/libinterface.a:
	gcc -c src/interface_win32.c -o obj/interface.o
	ar rcs lib/libinterface.a obj/interface.o

obj/ub3rparse.tab.c:
	bison --defines=obj/y.tab.h -o obj/ub3rparse.tab.c src/ub3rparse.y

obj/lex.yy.c:
	flex -o obj/lex.yy.c src/ub3rparse.lex

#win32: obj/ub3rparse.res lib/libinterface.a obj/ub3rparse.tab.c obj/lex.yy.c
win32: obj/ub3rparse.res lib/libinterface.a
	#gcc obj/ub3rparse.res src/main.c obj/lex.yy.c obj/ub3rparse.tab.c -o bin/ub3rparse.exe -L./lib -linterface -Wall -g
	gcc obj/ub3rparse.res src/main.c -o bin/ub3rparse.exe -L./lib -linterface -Wall -g

clean:
	rm bin/* -f
	rm lib/* -f 
	rm obj/* -f 
	touch bin/.gitkeep
	touch lib/.gitkeep
	touch obj/.gitkeep