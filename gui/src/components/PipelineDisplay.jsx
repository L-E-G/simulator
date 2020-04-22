import React from "react";

import styled from "styled-components";

import Table from "react-bootstrap/Table";

import { Badge } from "./styled";

const PipelineDiv = styled.div`
margin-left: 1rem;
`;

const PipelineDisplay = (props) => {
	let pipeline = props.pipeline;
	
	console.log(pipeline);

    for (var key in pipeline) {
	   if (pipeline[key] === "None") {
		  pipeline[key] = "Empty";
	   }
    }

    const statuses = Object.keys(pipeline).map((key) => {
	   return (
		  <td key={key}>
			 <h3><Badge variant="secondary">
				{pipeline[key]}
			 </Badge></h3>
		  </td>
	   );
	});
	
    return (
	   <PipelineDiv>
		  <h3>Pipeline</h3>

		  <Table bordered>
			 <thead>
				<tr>
				    <th>Cycle #</th>
				    <th>Fetch</th>
				    <th>Decode</th>
				    <th>Execute</th>
				    <th>Access Memory</th>
				    <th>Write Back</th>
				</tr>
			 </thead>
			 <tbody>
				<tr>
				    {statuses}
				</tr>
			 </tbody>
		  </Table>
	   </PipelineDiv>
    );
};

export default PipelineDisplay;
