import React, { useState } from "react";

import styled from "styled-components";

import Table from "react-bootstrap/Table";

import { Badge } from "./styled";

const PipelineDiv = styled.div`
margin-left: 1rem;
margin-right: 1rem;
`;

const NoPipelineBadgeContainer = styled.div`
text-align: center;
`;

const PipelineRow = (props) => {
    let pipeline = props.pipeline;

    return (
	   <td>
		  {Object.keys(pipeline).map((key) => {
			 return <StatusBadge status={pipeline[key]} />;
		  })}
	   </td>
    );
};

const StatusBadge = (props) => {
    let status = props.status;

    return (
	   <td>
		  <h3><Badge>
			 {status || "Empty"}
		  </Badge></h3>
	   </td>
    );
};

const PipelineDisplay = (props) => {
    let pipelines = props.pipelines;

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
				{pipelines.map((item, i) => (
				    <tr key={i}>
				    <td>{pipelines.length - i -1 }</td>
					   {Object.keys(item).map(key => (
						  
						  <td key={`pipeline-cycle-${i}-${key}`}>
							 <h3><Badge>
								{item[key] || "Empty"}
							 </Badge></h3>
						  </td>
					   ))}
				    </tr>
				))}
			 </tbody>
		  </Table>

		  {pipelines.length === 0 ?
		   <NoPipelineBadgeContainer>
			  <h3><Badge>No Pipeline Data</Badge></h3>
		   </NoPipelineBadgeContainer> : null}
	   </PipelineDiv>
    );
};

export default PipelineDisplay;
