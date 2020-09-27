# Parsley

## Description
Parsley is a command line utility that allows the user to specify fields to parse from input and output as a structured data format

## Usage
Parse the fields `name` and `age` and output as json. The use of the explicit anonymous capture is only required if the pattern includes a ':'
```
>>> parsley "name:[^ ]+" "_::" "age:.*" --json <<INPUT
... Sean:100
... INPUT
{"name":"Sean","age":"100"}
```

Parse the output of ls -l into csv format
```
>>> ls -l
total 28
-rw-r--r--. 1 sean sean 6354 Sep 26 18:53 Cargo.lock
-rw-r--r--. 1 sean sean  290 Sep 26 18:53 Cargo.toml
-rw-r--r--. 1 sean sean 1068 Sep 18 19:36 LICENSE
-rw-r--r--. 1 sean sean  756 Sep 26 18:33 README.md
drwxr-xr-x. 3 sean sean 4096 Sep 26 18:33 src
drwxr-xr-x. 4 sean sean 4096 Sep 18 19:39 target

>>> ls -l | ./target/debug/parsley --csv "permissions:[^ ]+" " " "hardlinks:[^ ]+" \
...                               " " "user:[^ ]+" " " "group:[^ ]+" " +" \
...                               "size:[^ ]+" " " "datetime:[A-Z][a-z]+ [0-9]{2} [0-9]{2}:[0-9]{2}" \
...                               " " "filename:.*"
permissions,hardlinks,user,group,size,datetime,filename
-rw-r--r--.,1,sean,sean,6354,Sep 26 18:53,Cargo.lock
-rw-r--r--.,1,sean,sean,290,Sep 26 18:53,Cargo.toml
-rw-r--r--.,1,sean,sean,1068,Sep 18 19:36,LICENSE
-rw-r--r--.,1,sean,sean,1085,Sep 26 20:40,README.md
drwxr-xr-x.,3,sean,sean,4096,Sep 26 18:33,src
drwxr-xr-x.,4,sean,sean,4096,Sep 18 19:39,target
```