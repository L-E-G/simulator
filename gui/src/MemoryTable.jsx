import React, { useState } from "react";

import Table from "react-bootstrap/Table";
import Badge from "react-bootstrap/Badge";
import Nav from "react-bootstrap/Nav";
import Button from "react-bootstrap/Button";
import ButtonGroup from "react-bootstrap/ButtonGroup";
import Form from "react-bootstrap/Form";

import ToggleExpandButton from "./ToggleExpandButton.jsx";

import "./MemoryTable.scss";

const MemoryTable = (props) => {
    let title = props.title;
    let memory = props.memory;

    const [expanded, setExpanded] = useState(true);
    
    const [search, setSearch] = useState("");
    const [searchAddrs, setSearchAddrs] = useState(true);
    const [searchVals, setSearchVals] = useState(false);

    // Toggle table
    const doToggleExpand = () => {
	   setExpanded(!expanded);
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

    // Format value based on selection
    const FMT_BIN = "Binary";
    const FMT_HEX = "Hex";
    const FMT_DECIMAL = "Decimal";

    const [format, setFormat] = useState(FMT_HEX);

    const binaryVariant = format === FMT_BIN ? 'primary' : 'outline-primary';
    const hexVariant = format === FMT_HEX ? 'primary' : 'outline-primary';
    const decimalVariant = format === FMT_DECIMAL ? 'primary' : 'outline-primary';

    var valueHeader = "Value"

    if (format == FMT_BIN) {
	   valueHeader += " (MSB ... LSB)";
    }

    var formatVal = (v) => v;

    if (format === FMT_BIN) {
	   // From: https://stackoverflow.com/a/16155417
	   formatVal = (v) => {
		  var str = (v >>> 0).toString(2);
		  while (str.length < 32) {
			 str = "0" + str;
		  }

		  return str;
	   };
    } else if (format === FMT_HEX) {
	   formatVal = (v) => "0x" + parseInt(v, 16);
    }
    
    const onFormatClick = (e) => {
	   setFormat(e.target.innerText);
    };

    // Table rows
    var filteredMemory = [];

    for (var key in memory) {
	   let keyStr = String(key);

	   if (keyStr.indexOf(search) !== -1) {
		  filteredMemory.push({
			 address: key,
			 value: memory[key],
		  });
	   }
    }
    
    const MemTableItems = Object.keys(filteredMemory).map((key) => {
	   return (
		  <tr key={filteredMemory[key].address}>
			 <td>{filteredMemory[key].address}</td>
			 <td>{formatVal(filteredMemory[key].value)}</td>
		  </tr>
	   );
    });

    // Make addresses searchable
    const onSearchChange = (e) => {
	   setSearch(e.target.value);
    };

    const onSearchAddrsChange = (e) => {
	   let val = e.target.value;

	   // Make so at least addresses or values are being searched
	   if (!val && !searchVals) {
		  return;
	   }
	   
	   setSearchAddrs(val);
    };

    const onSearchValsChange = (e) => {
	   let val = e.target.value;

	   // Make so at least addresses or values are being searched
	   if (!val && !searchAddrs) {
		  return;
	   }
	   
	   setSearchVals(val);
    };

    if (!expanded) {
	   return (
		  <div>
			 <ToggleExpandButton className="mem-table-toggle"
							 expanded={expanded}
							 doToggleExpand={doToggleExpand} />
			 <h3 className="mem-table-title">{title}</h3>
		  </div>
	   );
    }

    return (
	   <div>
		  <div>
			 <ToggleExpandButton className="mem-table-toggle"
					   expanded={expanded}
					   doToggleExpand={doToggleExpand} />
			 <h3 className="mem-table-title">{title}</h3>

			 <Nav className="mem-table-filters justify-content-end">
				<Nav.Item>
				    <Form>
					   <Form.Group controlId={"mem-table-search-" +
										 title.split(" ").join("-")}>
						  <Form.Control 
							 className="mem-table-search"
							 type="text"
							 placeholder={"Search " + title}
							 defaultValue={search}
							 onChange={onSearchChange} />
					   </Form.Group>
				    </Form>
				</Nav.Item>
				
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
		  </div>

		  <Table striped bordered>
			 <thead>
				<tr>
				    <th>Address</th>
				    <th>{valueHeader}</th>
				</tr>
			 </thead>
			 <tbody>
				{MemTableItems}
			 </tbody>
		  </Table>
		  <EmptyTableMsg />
	   </div>
    );
};

export default MemoryTable;
