import React, { useState, useContext } from "react";

import styled from "styled-components";

import Form from "react-bootstrap/Form";
import Card from "react-bootstrap/Card";
import Spinner from "react-bootstrap/Spinner";

import { SimulatorContext, ErrorContext } from "./App.jsx";
import ToggleExpandButton from "./ToggleExpandButton.jsx";

import { colors } from "../styles";

const UploadCard = styled(Card)`
width: 15rem;
margin: 1rem;
`;

const UploadToggleButton = styled(ToggleExpandButton)`
float: right;
`;

const UploadCardTitle = styled(Card.Title)`
margin-bottom: 0;
`;

const inputHeight = "2.3rem";
const UploadForm = styled(Form)`
height: ${inputHeight};
`;

const UploadFileInput = styled(Form.File)`
& > .custom-file-input {
    cursor: pointer;
}

& > .custom-file-label {
    text-align: center;
}

& > .custom-file-label::after {
    display: none;
}
`;

const InitialFileInput = styled(UploadFileInput)`
& > .custom-file-label {
    background: ${colors.primary};
    color: white;
}
`;

const ReuploadFileInput = styled(UploadFileInput)`
& > .custom-file-label {
    background: ${colors.red};
}
`;

const UploadMemFileForm = (props) => {
    const simulator = useContext(SimulatorContext);
    const setError = useContext(ErrorContext)[1];

    const [expanded, setExpanded] = useState(true);
    const [fileLoading, setFileLoading] = useState(false);
    const [fileSelected, setFileSelected] = useState(false);
    
    var reader = new FileReader();

    reader.onload = () => {
	   try {
		  var array = new Uint8Array(reader.result);
		  simulator.set_dram(array);
		  props.setDRAM(simulator.get_dram());

		  setFileLoading(false);
		  setExpanded(false);
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
			 <div>
				<InitialFileInput
				    label="Select File"
				    onChange={onFileChange}
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
				<div>
				    <ReuploadFileInput
					   label="Load Another"
					   onChange={onFileChange}
				        custom />
				</div>
			 );
		  }
	   }
    };

    const doToggleExpand = () => {
	   setExpanded(!expanded);
    };
    
    return (
	   <UploadCard>
		  <Card.Body>
			 <UploadCardTitle>
				<span>Memory File</span>

				<UploadToggleButton
				    expanded={expanded}
				    doToggleExpand={doToggleExpand}/>
			 </UploadCardTitle>

			 {expanded &&
				<div>
				    <Card.Body>
					   Upload a file to set the contents of simulator memory.
				    </Card.Body>
				    
				    <UploadForm>
					   <FormContents />
				    </UploadForm>
				</div>
			 }
		  </Card.Body>
	   </UploadCard>
    );
};

export default UploadMemFileForm;
