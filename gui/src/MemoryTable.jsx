import React, { useState } from "react";

import Table from "react-bootstrap/Table";
import Badge from "react-bootstrap/Badge";
import Nav from "react-bootstrap/Nav";
import Button from "react-bootstrap/Button";
import ButtonGroup from "react-bootstrap/ButtonGroup";

import "./MemoryTable.scss";

const MemoryTable = (props) => {
    // Value format select
    const FMT_BIN = "Binary";
    const FMT_HEX = "Hex";
    const FMT_DECIMAL = "Decimal";

    const [format, setFormat] = useState(FMT_HEX);

    const binaryVariant = format === FMT_BIN ? 'primary' : 'outline-primary';
    const hexVariant = format === FMT_HEX ? 'primary' : 'outline-primary';
    const decimalVariant = format === FMT_DECIMAL ? 'primary' : 'outline-primary';

    var formatVal = (v) => v;

    if (format === FMT_BIN) {
	   // From: https://stackoverflow.com/a/16155417
	   formatVal = (v) => (v >>> 0).toString(2);
    } else if (format === FMT_HEX) {
	   formatVal = (v) => "0x" + parseInt(v, 16);
    }
    
    const onFormatClick = (e) => {
	   setFormat(e.target.innerText);
    };
    
    // Table rows
    let memory = props.memory;
    
    const MemTableItems = () => {
	   let keys = Object.keys(memory);
	   
	   if (keys.length > 0) {
		  return keys.map((key) => (
			 <tr key={key}>
				<td>{key}</td>
				<td>{formatVal(memory[key])}</td>
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
		  <Nav className="mem-table-display-select justify-content-end">
			 <Nav.Item className="mem-table-display-select-label">
				Value Format
			 </Nav.Item>

			 <Nav.Item>
				<ButtonGroup onClick={onFormatClick}>
				    <Button variant={decimalVariant}>{FMT_DECIMAL}</Button>
				    <Button variant={hexVariant}>{FMT_HEX}</Button>
				    <Button variant={binaryVariant}>{FMT_BIN}</Button>
				</ButtonGroup>
			 </Nav.Item>
		  </Nav>
		  
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
