- Docker compose on fresh install errors for express, probably because mongodb takes time to start
- Check for connection resilience (e.g. when db is off and api does not crash, can continue working when db is back on). 
    Check that systems deals with Results and Errors rather than discards errors turning into Option
