# Parsley

## Description
Parsley is a command line utility that allows the user to specify fields to parse from input and output as a structured data format

## Usage
Parse the fields `name` and `age` and output as json. The use of the explicit anonymous capture is only required if the pattern includes a ':'
```
$ parsley "name:[^ ]+" "_::" "age:.*" --format json <<INPUT
... Sean:100
... INPUT
{"name":"Sean","age":"100"}
```

Parse the output of ls -l into csv format
```
$ ls -l
total 28
-rw-r--r-- 1 sean sean 6354 Oct 13  2020 Cargo.lock
-rw-r--r-- 1 sean sean  290 Oct 19  2020 Cargo.toml
-rw-r--r-- 1 sean sean 1068 Sep 18  2020 LICENSE
-rw-r--r-- 1 sean sean 1471 Oct 19  2020 README.md
drwxr-xr-x 3 sean sean 4096 Sep 23 19:09 src
drwxr-xr-x 4 sean sean 4096 Sep 18  2020 target

$ ls -l | target/debug/parsley --format csv "permissions:[^ ]+" " " "hardlinks:[^ ]+" \
        " " "user:[^ ]+" " " "group:[^ ]+" " +" \
        "size:[^ ]+" " " "datetime:[A-Z][a-z]+ [0-9]{2} ( [0-9]{4}|[0-9]{2}:[0-9]{2})" \
        " " "filename:.*""
permissions,hardlinks,user,group,size,datetime,filename
-rw-r--r--,1,sean,sean,6354,Oct 13  2020,Cargo.lock
-rw-r--r--,1,sean,sean,290,Oct 19  2020,Cargo.toml
-rw-r--r--,1,sean,sean,1068,Sep 18  2020,LICENSE
-rw-r--r--,1,sean,sean,1413,Oct 12 20:21,README.md
drwxr-xr-x,3,sean,sean,4096,Sep 23 19:09,src
drwxr-xr-x,4,sean,sean,4096,Sep 18  2020,target
```