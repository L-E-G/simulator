import React, { useState, useContext } from "react";
import Form from "react-bootstrap/Form";
import Button from "react-bootstrap/Button";
import Card from "react-bootstrap/Card";
import Spinner from "react-bootstrap/Spinner";

import { SimulatorContext, ErrorContext } from "./App.jsx";

import "./UploadMemFileForm.scss";

const UploadMemFileForm = (props) => {
    const simulator = useContext(SimulatorContext);
    const [error, setError] = useContext(ErrorContext);
    
    const [fileLoading, setFileLoading] = useState(false);
    const [fileSelected, setFileSelected] = useState(false);
    
    var reader = new FileReader();

    reader.onload = () => {
	   try {
		  var array = new Uint8Array(reader.result);
		  simulator.set_dram(array);
		  props.setDRAM(simulator.get_dram());

		  setFileLoading(false);
	   } catch(e) {
		  setError(e);
		  setFileSelected(false);
		  setFileLoading(false);
	   }
    };
    
    const onFileChange = (e) => {
	   setFileSelected(true);
	   setFileLoading(true);
	   
	   reader.readAsArrayBuffer(e.target.files[0]);
    };

    const FormContents = () => {
	   if (!fileSelected) {
		  return (
			 <div id="upload-mem-initial">
				<Form.File onChange={onFileChange}
						 custom />
			 </div>
		  );
	   } else {
		  if (fileLoading) {
			 return (
				<div>
				    <Spinner animation="grow" role="status" />
				    <span className="sr-only">Loading...</span>
				</div>
			 );
		  } else {
			 return (
				<div id="upload-mem-load-another">
				    <Form.File onChange={onFileChange}
				               custom />
				</div>
			 );
		  }
	   }
    };
    
    return (
	   <Card id="upload-mem-file">
		  <Card.Body>
			 <Card.Title>Memory File</Card.Title>

			 <Card.Body>
				Upload a file to set the contents of simulator memory.
			 </Card.Body>
			 
			 <Form id="upload-mem-form">
				<FormContents />
			 </Form>
		  </Card.Body>
	   </Card>
    );
};

export default UploadMemFileForm;
