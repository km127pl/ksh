0 READ output.txt
10 PRINT $READ
15 PRINT "Write content to the file"
20 INPUT $TEXT
25 WRITE output.txt $TEXT
27 READ output.txt
30 PRINT "New content" $READ