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

Parse the output of ls -l
```
ls -l | ./target/debug/parsley "permissions:[^ ]+" " " "hardlinks:[^ ]+" \
                               " " "user:[^ ]+" " " "group:[^ ]+" " +" \
                               "size:[^ ]+" " " "datetime:[A-Z][a-z]+ [0-9]{2} [0-9]{2}:[0-9]{2}" \
                               " " "filename:.*"
```