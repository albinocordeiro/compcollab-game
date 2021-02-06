## General plan
* Tool for downloading data (done)
* Setup database for storing market data
* Write api for populating data base and query by period 
* Write a BackTester (reads market data and provides events that can be subscribed to by the strategy)
* Write basic strategy runner and a basic sample strategy
    * Strategy pools for market events and act on them
        1. tick event
    