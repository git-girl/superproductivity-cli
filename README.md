# EARLY DEV: superproductivity-cli

## Steps 
- get a thing running that can output json from superproductivity 
  - there is the 2 minute backup thing but then you cant write to the data and get reads only
  - [ ] understand how the three syncing methods work and how you can hook into them
    - [ ] sync file 
    - [ ] sync webdav
    - [ ] sync dropbox
      - via some authcode
    - if attaching to them in some form isnt possible id need to block the sync point and 
      then reimplement all syncing features which i dont think is good

- get a thing running that parses that json 
  - represent the data in appropriate datastructures
- get a programm runing that can query those datastructures
- get a TUI running with ratatui that presents the datastrucutres 
- get something running that manips the tasks etc.
- ... 

## Goals

- Providing a scriptable interface for Superproductivity 
  - reading the data
  - writing to the data
- Not taking up any API endpoints ( more specifically blocking the sync slot )

## Non Goals 

- being a full feature replacement for the gui app
  - while that would be nice its not something thats realistic atm

## Tools to use

  Purpose   Tool
  --------- -------------------------------------------
  TUI Lib   https://github.com/tui-rs-revival/ratatui
  
