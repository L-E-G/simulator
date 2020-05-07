import React, { useState } from "react";

import styled from "styled-components";

import Table from "react-bootstrap/Table";
import Nav from "react-bootstrap/Nav";
import Form from "react-bootstrap/Form";
import Dropdown from "react-bootstrap/Dropdown";

import settingsIcon from "../images/settings.png";

import ToggleExpandButton from "./ToggleExpandButton.jsx";
import { colors } from "../styles";
import { Badge, DropdownToggle } from "./styled";
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

const SearchSettingsToggle = styled(DropdownToggle)`
&.dropdown-toggle {
    border: none;
    border-left: 1px solid ${colors.primary};
    border-radius: 0;
    display: inline-block;
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

const ValueHeader = styled.div`
display: flex;
`;

const ValueHeaderLabel = styled.span`
flex-grow: 1;
align-self: end;
`;

const FormatSettingsDropdown = styled(Dropdown)`
& img {
    width: 1.5rem;
}
`;

const EmptyMsgTxt = styled.div`
text-align: center;
`;

const MemoryTable = (props) => {
    const title = props.title;
    const memory = props.memory;
    const keyAliases = props.keyAliases || {};
    const addressesColName = props.addressesColName || "Addresses";
    const highlightedAddress = props.highlightedAddress ?
						 Number(props.highlightedAddress) : null;

    const [expanded, setExpanded] = useState(true);
    
    const [search, setSearch] = useState("");
    const [searchAddrs, setSearchAddrs] = useState(true);
    const [searchVals, setSearchVals] = useState(false);
    const [searchFuzzy, setSearchFuzzy] = useState(true);

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

    // Filter table rows based on search input
    var filteredMemory = [];

    for (var key in memory) {
	   let keyStr = String(key);
	   let valueStr = String(formatVal(memory[key]));

	   let item = {
		  address: keyStr,
		  value: valueStr,
	   };

	   if (searchAddrs === true) {
		  if (searchFuzzy === true && keyStr.indexOf(search.toLocaleLowerCase()) !== -1) {
			 filteredMemory.push(item);
		  } else if (searchFuzzy === false && keyStr === search) {
			 filteredMemory.push(item);
		  } else if (searchFuzzy === true && key in keyAliases &&
				   keyAliases[key].toLocaleLowerCase().indexOf(search.toLocaleLowerCase()) !== -1) {
			 filteredMemory.push(item);
		  } else if (searchFuzzy === false && key in keyAliases &&
				   keyAliases[key] === search) {
			 filteredMemory.push(item);
		  }
	   } else if (searchVals === true) {
		  if (searchFuzzy === true && valueStr.indexOf(search) !== -1) {
			 filteredMemory.push(item);
		  } else if (searchFuzzy === false && valueStr === search) {
			 filteredMemory.push(item);
		  }
	   }
    }

    // Make addresses searchable
    const onSearchChange = (e) => {
	   setSearch(e.target.value);
    };

    const onSearchAddrsClick = (e) => {
	   let val = !searchAddrs;

	   // Make so at least addresses or values are being searched
	   if (!val && !searchVals) {
		  setSearchVals(true);
		  setSearchAddrs(false);
		  return;
	   }

	   setSearchAddrs(val);
    };

    const onSearchValsClick = (e) => {
	   let val = !searchVals;

	   // Make so at least addresses or values are being searched
	   if (!val && !searchAddrs) {
		  setSearchAddrs(true);
		  setSearchVals(false);
		  return;
	   }

	   setSearchVals(val);
    };

    const onSearchFuzzyClick = (e) => {
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
								    <CheckInput
									   value={searchAddrs}
									   onClick={onSearchAddrsClick}
									   label="Search Addresses" />
								</Dropdown.Item>
								
								<Dropdown.Item>
								    <CheckInput
									   value={searchVals}
									   onClick={onSearchValsClick}
									   label="Search Values" />
								</Dropdown.Item>
								
								<Dropdown.Item>
								    <CheckInput
									   value={searchFuzzy}
									   onClick={onSearchFuzzyClick}
									   label="Fuzzy Search" />
								</Dropdown.Item>
							 </Dropdown.Menu>
						  </Dropdown>
					   </Form.Group>
				    </Form>
				</SearchContainer>
			 </FiltersNav>
		  </div>

		  <Table striped bordered>
			 <thead>
				<tr>
				    <th>{addressesColName}</th>
				    <th>
					   <ValueHeader>
						  <ValueHeaderLabel>{valueHeader}</ValueHeaderLabel>
						  <FormatSettingsDropdown>
							 <DropdownToggle>
								<img
								    alt="Value display settings"
								    src={settingsIcon} />
							 </DropdownToggle>

							 <Dropdown.Menu onClick={onFormatClick}>
								<Dropdown.Item active={decimalActive}>
								    {FMT_DECIMAL}
								</Dropdown.Item>
								<Dropdown.Item active={hexActive}>
								    {FMT_HEX}
								</Dropdown.Item>
								<Dropdown.Item active={binaryActive}>
								    {FMT_BIN}
								</Dropdown.Item>
							 </Dropdown.Menu>
						  </FormatSettingsDropdown>
					   </ValueHeader>
				    </th>
				</tr>
			 </thead>
			 <tbody>
				{Object.keys(filteredMemory).map((i) => {
				    let addr = filteredMemory[i].address;
				    let key = addr + (addr in keyAliases ?
								  " (" + keyAliases[addr] + ")" : "");
				    let value = filteredMemory[i].value;

				    var style = {};

				    if (addr === highlightedAddress) {
					   style = {
						  background: "yellow",
					   }
				    }
				    
				    return (
					   <tr
						  key={addr}
						  style={style}
					   >
						  <td>{key}</td>
						  <td>{value}</td>
					   </tr>
				    );
				})}
			 </tbody>
		  </Table>

		  {Object.keys(memory).length === 0 &&
		   <EmptyMsgTxt>
			  <h3>
				 <Badge>
					{title} empty
				 </Badge>
			  </h3>
	        </EmptyMsgTxt>
		  }
	   </div>
    );
};

export default MemoryTable;
