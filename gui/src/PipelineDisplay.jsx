import React from "react";

import Badge from "react-bootstrap/Badge";
import Table from "react-bootstrap/Table";

import "./PipelineDisplay.scss";

const PipelineDisplay = (props) => {
    let pipeline = props.pipeline;

    for (var key in pipeline) {
	   if (pipeline[key] === null) {
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
	   <div className="pipeline">
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
				    <td>0</td>
				    {statuses}
				</tr>
			 </tbody>
		  </Table>
	   </div>
    );
};

export default PipelineDisplay;
