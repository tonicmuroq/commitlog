### CLaaS (Commit Log as a Service)

受到 [这位哥](http://whatthecommit.com/index.txt) 的启发, 搞个中文版的...

### Configure your .gitconfig

```
$ cat ~/.gitconfig
[alias]
    wtf = !git commit -m"$(curl -L -s https://commitlog.wolege.ca)"

$ git wtf
[master (root-commit) 8a4796c] 打错字了
 1 file changed, 0 insertions(+), 0 deletions(-)
  create mode 100644 aaa
```
