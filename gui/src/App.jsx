import React from "react";
import Navbar from "react-bootstrap/Navbar";

import { Simulator } from "simulator";

import "./App.scss";

function App() {
    var simulator = new Simulator();

    console.log(simulator.get_dram_addresses());
    
    return (
	   <div className="app">
		  <Navbar id="header" bg="primary" expand="md">
			 <Navbar.Brand>
				<img src="/logo.png" />
				<span id="brand-name">LEG Simulator</span>
			 </Navbar.Brand>
		  </Navbar>
	   </div>
    );
}

export default App;
