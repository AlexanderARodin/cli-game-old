help:
	@cat Makefile

edit:
	@nvim src/main.rs

test:
	@cargo test
run:
	@cargo run -r -- -h

# # # # # # # #
pull:
	@git pull

savetogit: git.pushall
git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

# # # # # # # #
clean:
	@cargo clean
