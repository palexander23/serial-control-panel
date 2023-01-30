# Serial Control Panel
An auto-configuring utility for presenting a UI around the serial interface to an external system.

This project is under active development. 
The look and functionality are highly unstable.

## Motivation
I work with many multidisciplinary teams at my job.
I am often the only software/electronics engineer on a given project. 
When handing over prototypes to other team members with little/no coding experience I want to provide them with a UI that allows both "at a glance" intuitive thinking and in-built data logging.

I usually use the Arduino environment to achieve this. It has both plotting and CLI interfaces built in. 
However, as I write more software I find myself reimplementing the same patterns for the CLI: 
1) A constant-frequency printout of CSV data containing system state and sensor data.
2) A CLI taking single character commands with numerical arguments.
3) Debug, warning, and error messages printed amongst the CSV data tagged with `#` for easy filtering.

With these features in mind, a simple UI can be constructed allowing plotting, simple UI icons (readouts, dials, bars), log output, and filtered CSV saving. 

I don't want to have to update the software I distribute to colleagues every time I update my code, so this UI should automatically configure itself.
This should be done by having JSON on the embedded system describing the UI that the UI can request using a special CLI command. 


## Project Phases
This software will be developed in phases as a way of practicing this kind of development. These phases shall be as follows:

### Phase 1: Prototype System
A demo app built using the chosen GUI and serial libraries.  

**Core Tasks/Features:**
* Plotting 
* A single display UI element (Text box with label).
* A single control UI element (Input box with label).
* Serial comms to get/send data and reconfigure the UI.
* A configurable dummy serial device compatible with the prototype UI.

### Phase 2: Design
Using learnings from the prototype to define features/interfaces for version 1.0.

**Core Tasks/Features:**
* Define all display UI blocks (text box, dial, on-off indicator, etc.).
* Define all control UI blocks (input box, switch, slider?, etc.).
* Define protocol for input UI blocks (send and confirmation).
* Define JSON format for serialization.
* Define UI Layout.
* Define logging behavior

### Phase 3: v1.0 Implementation
Coding up the designs created in Phase 2 for the first usable release.

**Core Tasks/Features:**  
*TBC*
