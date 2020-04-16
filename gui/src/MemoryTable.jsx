import React from "react";

import Table from "react-bootstrap/Table";
import Badge from "react-bootstrap/Badge";

import "./MemoryTable.scss";

const MemoryTable = (props) => {
    let memory = props.memory;
    
    const MemTableItems = () => {
	   let keys = Object.keys(memory);
	   
	   if (keys.length > 0) {
		  return keys.map((key) => (
			 <tr key={key}>
				<td>{key}</td>
				<td>{memory[key]}</td>
			 </tr>
		  ));
	   }
	   
	   return null;
    };

    const EmptyTableMsg = () => {
	   if (Object.keys(memory).length === 0) {
		  return (
			 <div className="mem-table-empty-msg">
				<h3>
				    <Badge variant="secondary">
					   Memory empty
				    </Badge>
				</h3>
			 </div>
		  );
	   }
	   
	   return null;
    };

    return (
	   <div>
		  <Table striped bordered>
			 <thead>
				<tr>
				    <th>Address</th>
				    <th>Value</th>
				</tr>
			 </thead>
			 <tbody>
				<MemTableItems />
			 </tbody>
		  </Table>
		  <EmptyTableMsg />
	   </div>
    );
};

export default MemoryTable;
