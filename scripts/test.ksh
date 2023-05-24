#!"C:\Users\Jakub\Desktop\projects\rust-learn\04-client\target\release\ksh.exe"
0 VAR counter 0
1 VAR welcome "Hello!"
10 PRINT welcome
20 PRINT "What is your name?"
30 INPUT $name
40 WRITE name.txt $name
50 READ name.txt
60 PRINT "Hello," $READ
70 PRINT "This is a counter"
80 VAR counter 0
90 PRINT "It is:" counter
91 INC counter
92 PRINT "It just got incremented!"
93 PRINT "Its now" counter
