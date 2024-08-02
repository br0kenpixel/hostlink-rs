searchState.loadedDescShard("hostlink", 0, "Module for communicating with PLCs using Hostlink.\nContains implementations of the Hostlink protocol.\nNumber of bits per character\n8 bits per character\n5 bits per character\nFlow control modes\nFlow control using RTS/CTS signals.\nNo flow control.\nOne stop bit.\nA trait for serial port devices\nA struct containing all serial port settings\n7 bits per character\n6 bits per character\nFlow control using XON/XOFF bytes.\nNumber of stop bits\nTwo stop bits.\nReturns the current baud rate.\nSet the baud rate in symbols-per-second\nGets the number of bytes available to be read from the …\nGet the number of bytes written to the output buffer, …\nDiscards all bytes from the serial driver’s input buffer …\nStop transmitting a break\nReturns the character size.\nSet the number of bits used to represent a character sent …\nReturns the flow control mode.\nSet the type of signalling to use for controlling data …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the name of this port if it exists.\nOpen a cross-platform interface to the port with the …\nOpen a platform-specific interface to the port with the …\nReturns the parity-checking mode.\nSet the type of parity to use for error checking\nSet the path to the serial port\nReads the state of the Carrier Detect control signal.\nReads the state of the CTS (Clear To Send) control signal.\nReads the state of the Data Set Ready control signal.\nReads the state of the Ring Indicator control signal.\nSets the baud rate.\nStart transmitting a break\nSets the character size.\nSets the flow control mode.\nSets the parity-checking mode.\nSets the number of stop bits.\nSets the timeout for future I/O operations.\nReturns the number of stop bits.\nSet the number of bits to use to signal the end of a …\nReturns the current timeout.\nSet the amount of time to wait to receive data before …\nAttempts to clone the <code>SerialPort</code>. This allow you to write …\nWrites to the Data Terminal Ready pin\nSets the state of the RTS (Request To Send) control signal.\nA simplified representation of a command.\nInvalid Node ID.\nFailed to parse string as integer.\nTest command’s message block contains invalid characters.\nA complete Hostlink command.\nA Hostlink command type.\nStores a command’s parameters as ASCII values.\nMissing ‘@’ symbol at start.\nMissing FCS checksum.\nMissing header code (command code).\nInvalid or missing Node ID.\nMissing command terminator.\nRepresents a Node ID - i.e. a number between 0 and 99.\nRepresents a protocol error.\nReads the operating status of the PLC.\nTransmits a block of data, which is then repeated by the …\nUnknown or unsupported command type.\nReturns the command code.\nFCS Checksum calculation and types.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPerform conversion into <code>Message</code>.\nGet the command’s message type.\nConstruct a <code>StatusRead</code> command.\nConstruct a <code>Test</code> command with the given data.\nCreates an empty argument set.\nSafely constructs a Node ID from the specified value. If …\nCreates a new command from the specified node ID and …\nConstructs a Node ID <strong>without</strong> verifying the actual value.\nCreates a new command with no arguments from the specified …\nResponse types.\nSerializes the command into a string that can be sent to a …\nFCS Checksum bytes.\nCalculates the FCS checksum from a serialized Hostlink …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the FCS checksum bytes as a single numeric value.\nResponse types for the <code>StatusRead</code> command.\nMissing memory status bytes\nMissing mode bytes\nRepresents the status of a PLC device.\nMemory status.\nAn operation mode.\nAn error that can occur while trying to parse <code>Status</code>.\nMemory size bits could not be mapped to any known memory …\nMode bits could not be mapped to any known operation mode\nMessage contains an error\nFatal error generated\nFALS <em>(Failure Alarm And Reset)</em> generated\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMemory status\nOperation mode\nParse the operation mode from a byte obtained using a …\nParse the memory status from a byte obtained using a …\nSize of program memory in bytes (if available).\nWhether the program memory is write protected.")