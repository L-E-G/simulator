import React from "react";

import { Simulator } from "simulator";

import "./App.scss";

function App() {
    var simulator = new Simulator();

    console.log(simulator.get_addresses());
    
    return (
	   <div className="app">
		  Hello world
	   </div>
    );
}

export default App;
