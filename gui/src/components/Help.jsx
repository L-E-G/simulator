import React, { useState } from "react";

import styled from "styled-components";

import Modal from "react-bootstrap/Modal";

import { SecondaryButton } from "./styled";

import helpIcon from "../images/help.png";
import loadExampleFileScreenshot from "../images/help-screenshots/load-example-file.png";
import loadCustomFileScreenshot from "../images/help-screenshots/load-custom-file.png";
import runStepButtonsScreenshot from "../images/help-screenshots/run-step-buttons.png";
import loadSameFileCheckScreenshot from "../images/help-screenshots/load-same-file-check.png";
import pipelineRecentOnlyScreenshot from "../images/help-screenshots/pipeline-recent-only.png";
import memoryValueFormatScreenshot from "../images/help-screenshots/memory-value-format.png";
import memorySearchScreenshot from "../images/help-screenshots/memory-search.png";


const HelpContainer = styled.div`
position: fixed;
bottom: 1rem;
right: 1rem;
z-index: 2;
`;

const HelpImg = styled.img`
width: 2.5rem;
height: 2.5rem;
margin-right: 0.5rem;
`;

const HelpTitle = styled.span`
font-size: 1.3rem;
font-weight: bold;
`;

const ScreenshotImg = styled.img`
display: block;
max-width: 15rem;
border: 0.1rem solid rgba(0,0,0,.1);
border-radius: 0.1rem;
padding: 1rem;
margin-top: 1rem;
margin-bottom: 1rem;
`;

const HelpSection = styled.h4`
margin-top: 2rem;
`;

const Help = (props) => {
    const [show, setShow] = useState(false);

    const handleOpen = () => {
	   setShow(!show);
    };

    const handleClose = () => {
	   setShow(false);
    };
    
    return (
	   <HelpContainer {...props}>
		  <SecondaryButton onClick={handleOpen}>
			 <HelpImg src={helpIcon} alt="Help icon" />
			 <HelpTitle>Help</HelpTitle>
		  </SecondaryButton>

		  <Modal show={show} onHide={handleClose}>
			 <Modal.Header closeButton>
				<Modal.Title>
				    <HelpImg src={helpIcon} alt="Help icon" />
				    Simulator Help
				</Modal.Title>
			 </Modal.Header>

			 <Modal.Body>
				This is a simulation of a computer that implements the &nbsp;
				<a href="https://l-e-g.github.io">
				    L.E.G. instruction set architecture
				</a>
				.
				<br /><br />
				To use the simulator:
				<ol>
				    <li>
					   Select an example memory file. This loads a program
					   into memory.
					   <ScreenshotImg
						  src={loadExampleFileScreenshot}
						  alt="Screenshot of load example memory file UI" />
				    </li>
				    <li>
					   Click the <i>Run</i> or <i>Step</i> buttons to execute
					   the program.
					   <br /><br />
					   The <i>Run</i> button runs the program in
					   memory to completion.
					   <br /><br />
					   While the <i>Step</i> button
					   executes the next instruction in the simulator.

					   <ScreenshotImg
						  src={runStepButtonsScreenshot}
						  alt="Screenshot of run and step buttons" />
				    </li>
				</ol>
				<HelpSection>Memory File Interface</HelpSection>
				The <b>Memory File</b> section of the user interface allows you
				to load the contents of a binary file into the
				simulator's DRAM.
				<br /><br />
				If you have your own LEG binary file you may load it with
				the select file menu.
				<ScreenshotImg
				    src={loadCustomFileScreenshot}
				    alt="Screenshot of load custom memory file UI" />

				The <i>Load the same file in the future</i> check box
				will automatically load the same memory file which you just
				selected whenever the page loads in the future.
				<ScreenshotImg
				    src={loadSameFileCheckScreenshot}
				    alt="Screenshot of load same memory file check box" />

				<HelpSection>Pipeline Interface</HelpSection>
				The <b>Pipeline</b> section of the user interface displays the
				instructions in each stage of the simulator's pipeline.
				<br /><br />
				The <i>Only Show Most Recent</i> check box sets if the
				pipeline interface should only show the instructions in the
				most recent step of the program execution.
				<ScreenshotImg
				    src={pipelineRecentOnlyScreenshot}
				    alt="Screenshot of only show most recent check box" />

				<HelpSection>Memory View Interfaces</HelpSection>
				The <b>Registers</b> and <b>DRAM</b> sections of the user
				interface show the contents of the simulator's registers and
				DRAM correspondingly.
				<br /><br />
				The format of values can be changed by clicking the settings
				gear button in the <i>Value</i> header row.
				<ScreenshotImg
				    src={memoryValueFormatScreenshot}
				    alt="Screenshot of memory value format dropdown" />

				The search box can also be used to filter the contents
				of these displays. Click the gear button in the search box
				to change how the search box works.
				<ScreenshotImg
				    src={memorySearchScreenshot}
				    alt="Screenshot of memory search bar and settings dropdown"
				    />
			 </Modal.Body>
		  </Modal>
	   </HelpContainer>
    );
};

export default Help;
