import React, { useState, useContext, useEffect } from "react";

import Navbar from "react-bootstrap/Navbar";
import Toast from "react-bootstrap/Toast";
import Button from "react-bootstrap/Button";

import { Simulator } from "simulator";

import MemoryTable from "./MemoryTable.jsx";
import UploadMemFileForm from "./UploadMemFileForm.jsx";
import PipelineDisplay from "./PipelineDisplay.jsx";

import "./App.scss";

const SimulatorContext = React.createContext(null);
const ErrorContext = React.createContext([{}, () => {}]);

const Error = () => {
    const [error, setError] = useContext(ErrorContext);

    if (error !== null) {
	   console.error("App error:", error);

	   // Convert error into string, ensure first letter is uppercase
	   var strError = String(error);
	   strError = strError.charAt(0).toUpperCase() + strError.slice(1);
	   
	   const doClose = () => {
		  setError(null);
	   };

	   return (
		  <Toast id="error-toast"
			    onClose={doClose}>
			 <Toast.Header>
				<img src="/error.png"
				     id="error-icon"
				     className="rounded mr-2"
				     alt="Error icon" />
				
				<strong className="mr-auto">
				    Error
				</strong>
			 </Toast.Header>
			 <Toast.Body>
				{strError}
			 </Toast.Body>
		  </Toast>
	   );
    }
    
    return null;
};

const App = () => {
    var simulator = new Simulator();

    const [registers, setRegisters] = useState(simulator.get_registers());
    const [dram, setDRAM] = useState(simulator.get_dram());
    const [pipeline, setPipeline] = useState(simulator.get_pipeline());
    const [error, setError] = useState(null);
    
    const onStepClick = () => {
	   try {
		  simulator.step();

		  setPipeline(simulator.get_registers());
		  setDRAM(simulator.get_dram());
		  setPipeline(simulator.get_pipeline());
	   } catch (e) {
		  setError(e);
	   }
    };
    
    return (
	   <div className="app">
		  <ErrorContext.Provider value={[error, setError]}>
			 <SimulatorContext.Provider value={simulator}>
				<Navbar id="header" bg="primary" expand="md">
				    <Navbar.Brand>
					   <img src="/logo.png" alt="LEG computer logo" />
					   <span id="brand-name">LEG Simulator</span>
				    </Navbar.Brand>

				    <Navbar.Collapse className="justify-content-end">
					   <Navbar.Text>
						  <Button id="step-button"
								variant="secondary"
								onClick={onStepClick}>
							 Step
						  </Button>
					   </Navbar.Text>
				    </Navbar.Collapse>
				</Navbar>

				<Error />

				<UploadMemFileForm setDRAM={setDRAM} />

				<PipelineDisplay pipeline={pipeline} />

				<MemoryTable title="Registers" memory={registers} />
				<MemoryTable title="DRAM" memory={dram} />
			 </SimulatorContext.Provider>
		  </ErrorContext.Provider>
	   </div>
    );
};

export default App;
export { SimulatorContext, ErrorContext };
