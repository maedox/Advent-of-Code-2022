/A/ {if ($2 == "X") a += 3; else if ($2 == "Y") a += 4; else a += 8}
/B/ {if ($2 == "X") a += 1; else if ($2 == "Y") a += 5; else a += 9}
/C/ {if ($2 == "X") a += 2; else if ($2 == "Y") a += 6; else a += 7}
END {print "Part 2: ",  a}

