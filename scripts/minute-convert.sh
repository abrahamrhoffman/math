#!/bin/bash
######################
# Seconds to Minutes #
########################
# Author : Abe Hoffman ##
# Date   : June 14 2018 #
#########################

# Rationale: 6m12s is easier to understand than 6.20m

# Start with a total number of whole or partial seconds
SECONDS=`expr $1` # Informs bash to treat the user's input as an integer

# Convert to minutes and store the whole number portion
MINUTES=`bc <<< "scale=2;$SECONDS/60" | awk -F\. '{print $1}'`

# Convert to minutes and store the decimal portion
MINUTESDECIMAL=`bc <<< "scale=2;$SECONDS/60" | awk -F\. '{print $2}'`

# Since we now have our whole number minutes and decimal minutes stored,
# we only need to convert the decimal minutes into seconds. This provides
# nicer insight for the user.

# Multiply the remaining decimal minutes by 60 to convert to seconds
# then divide by 100 to approximate within 10e^2 the number of remaining seconds
SECS=`bc <<< "scale=2;($MINUTESDECIMAL*60)/100" | awk -F\. '{print $1}'`

# Print the result to the screen
echo ${MINUTES}m${SECS}s
