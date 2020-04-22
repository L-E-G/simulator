import React, { useState } from "react";

import styled, { css } from "styled-components";

import Table from "react-bootstrap/Table";
import Nav from "react-bootstrap/Nav";
import Button from "react-bootstrap/Button";
import ButtonGroup from "react-bootstrap/ButtonGroup";
import Form from "react-bootstrap/Form";
import Dropdown from "react-bootstrap/Dropdown";

import settingsIcon from "../images/settings.png";

import ToggleExpandButton from "./ToggleExpandButton.jsx";
import { colors } from "../styles";
import { OutlinedButton, Badge } from "./styled";
import CheckInput from "./CheckInput";

const MemToggleExpandButton = styled(ToggleExpandButton)`
margin-bottom: 0.5rem;
margin-left: 1rem;
`;

const MemTableTitle = styled.h3`
display: inline-block;
margin-top: 0.2rem;
margin-left: 1rem;
`;

const FiltersTitle = styled.span`
margin-right: 1rem;
margin-top: 0.4rem;
`;

const FiltersNav = styled(Nav)`
margin-bottom: 1rem !important;
float: right;
justify-content: center;

& > .nav-item {
    margin: 0.5rem;
}
`;

const SearchContainer = styled(Nav.Item)`
margin-right: 1rem;
border: 1px solid ${colors.primary};
border-radius: 0.25rem;
`;

const SearchSettingsToggle = styled(Dropdown.Toggle)`
&.dropdown-toggle {
    border: none;
    border-left: 1px solid ${colors.primary};
    border-radius: 0;
    display: inline-block;
    background: none;
    color: ${colors.primary};
}

.show > &, &:hover {
    background: ${colors.primary} !important;
}
`;

const SearchSettingsImg = styled.img`
width: 1.5rem;
`;

const SearchInput = styled(Form.Control)`
max-width: 20rem;
display: inline-block;
border: none;
margin: auto;
`;

const FormatButtonGroup = styled(ButtonGroup)`
margin-right: 1rem;
`;

const EmptyMsgTxt = styled.div`
text-align: center;
`;

const MemoryTable = (props) => {
    let title = props.title;
    let memory = props.memory;

    const [expanded, setExpanded] = useState(true);
    
    const [search, setSearch] = useState("");
    const [searchAddrs, setSearchAddrs] = useState(true);
    const [searchVals, setSearchVals] = useState(false);
    const [searchFuzzy, setSearchFuzzy] = useState(true);
    console.log("addrs=" + searchAddrs + ", vals=" + searchVals +
			 ", fuzzy=" + searchFuzzy);

    // TODO: Debug why search filter check boxes aren't setting

    // Toggle table
    const doToggleExpand = () => {
	   setExpanded(!expanded);
    };

    // Format value based on selection
    const FMT_BIN = "Binary";
    const FMT_HEX = "Hex";
    const FMT_DECIMAL = "Decimal";

    const [format, setFormat] = useState(FMT_HEX);

    const binaryActive = format === FMT_BIN || null;
    const hexActive = format === FMT_HEX || null;
    const decimalActive = format === FMT_DECIMAL || null;

    var valueHeader = "Value"

    if (format === FMT_BIN) {
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
	   let valueStr = String(formatVal(memory[key]));

	   let item = {
		  address: keyStr,
		  value: valueStr,
	   };

	   if (searchAddrs) {
		  if (searchFuzzy && keyStr.indexOf(search) !== -1) {
			 filteredMemory.push(item);
		  } else if (!searchFuzzy && keyStr === search) {
			 filteredMemory.push(item);
		  }
	   } else if (searchVals) {
		  if (searchFuzzy && valueStr.indexOf(search) !== -1) {
			 filteredMemory.push(item);
		  } else if (!searchFuzzy && valueStr === search) {
			 filteredMemory.push(item);
		  }
	   }
    }
    
    const MemTableItems = Object.keys(filteredMemory).map((key) => {
	   let addr = filteredMemory[key].address;
	   let value = filteredMemory[key].value;
	   
	   return (
		  <tr key={addr}>
			 <td>{addr}</td>
			 <td>{value}</td>
		  </tr>
	   );
    });

    // Make addresses searchable
    const onSearchChange = (e) => {
	   setSearch(e.target.value);
    };

    const onSearchAddrsClick = (e) => {
	   let val = !searchAddrs;

	   // Make so at least addresses or values are being searched
	   if (!val && !searchVals) {
		  console.log("addrs change, addrs=false, vals=true");
		  setSearchVals(true);
		  setSearchAddrs(false);
		  return;
	   }

	   console.log("addrs change, addrs=", val);
	   setSearchAddrs(val);
    };

    const onSearchValsClick = (e) => {
	   let val = !searchVals;

	   // Make so at least addresses or values are being searched
	   if (!val && !searchAddrs) {
		  console.log("vals change, addrs=true, vals=false");
		  setSearchAddrs(true);
		  setSearchVals(false);
		  return;
	   }

	   console.log("vals change, vals=", val);
	   setSearchVals(val);
    };

    const onSearchFuzzyClick = (e) => {
	   console.log("fuzzy change, fuzzy=", !searchFuzzy);
	   setSearchFuzzy(!searchFuzzy);
    };

    if (!expanded) {
	   return (
		  <div>
			 <MemToggleExpandButton
				expanded={expanded}
				doToggleExpand={doToggleExpand} />
			 <MemTableTitle>{title}</MemTableTitle>
		  </div>
	   );
    }

    const ctrlIdPart = title.split(" ").join("-");

    const onSearchSubmit = (e) => {
	   e.preventDefault();
	   e.stopPropagation();
    };

    return (
	   <div>
		  <div>
			 <MemToggleExpandButton
				expanded={expanded}
				doToggleExpand={doToggleExpand} />
			 <MemTableTitle>{title}</MemTableTitle>

			 <FiltersNav>
				<SearchContainer>
				    <Form onSubmit={onSearchSubmit} inline>
					   <Form.Group
						  controlId={"mem-table-search-" + ctrlIdPart}>
						  <SearchInput
							 type="text"
							 placeholder={"Search " + title}
							 defaultValue={search}
							 onChange={onSearchChange} />

						  <Dropdown>
							 <SearchSettingsToggle>
								<SearchSettingsImg
								src={settingsIcon}
								alt="Search Setting Icon" />
							 </SearchSettingsToggle>
							 
							 <Dropdown.Menu>
								<Dropdown.Item>
								    <CheckInput value={searchAddrs}
											 onClick={onSearchAddrsClick}
											 label="Search Addresses" />
								</Dropdown.Item>
								
								<Dropdown.Item>
								    <CheckInput value={searchVals}
											 onClick={onSearchValsClick}
											 label="Search Values" />
								</Dropdown.Item>
								
								<Dropdown.Item>
								    <CheckInput value={searchFuzzy}
											 onClick={onSearchFuzzyClick}
											 label="Fuzzy Search" />
								</Dropdown.Item>
							 </Dropdown.Menu>
						  </Dropdown>
					   </Form.Group>
				    </Form>
				</SearchContainer>

				<Nav.Item>
				    <FiltersTitle>
					   Value Format
				    </FiltersTitle>
				    
				    <FormatButtonGroup onClick={onFormatClick}>
					   <OutlinedButton active={decimalActive}>
						  {FMT_DECIMAL}
					   </OutlinedButton>
					   <OutlinedButton active={hexActive}>
						  {FMT_HEX}
					   </OutlinedButton>
					   <OutlinedButton active={binaryActive}>
						  {FMT_BIN}
					   </OutlinedButton>
				    </FormatButtonGroup>
				</Nav.Item>
			 </FiltersNav>
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

		  {Object.keys(memory).length === 0 &&
		   <EmptyMsgTxt>
			  <h3>
				 <Badge variant="secondary">
					Memory empty
				 </Badge>
			  </h3>
	        </EmptyMsgTxt>
		  }
	   </div>
    );
};

export default MemoryTable;
