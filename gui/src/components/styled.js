import styled from "styled-components";

import BootstrapButton from "react-bootstrap/Button";
import BootstrapBadge from "react-bootstrap/Badge";
import Dropdown from "react-bootstrap/Dropdown";

import { colors } from "../styles";

const Color = require("color");

const styledButton = (mainColor, secondaryColor) => {
    return styled(BootstrapButton)`
background: ${mainColor};

&.active, &:hover {
    background: white;
    color: ${secondaryColor};
}

&:disabled {
    background: ${Color(mainColor).darken(0.5)};
    border: 1px solid ${Color(mainColor).darken(0.5)};
}
`;
};

const PrimaryButton = styledButton(colors.primary, colors.secondary);
const SecondaryButton = styledButton(colors.secondary, colors.primary);

const OutlinedButton = styled(BootstrapButton)`
background: white;
color: ${colors.primary};
border: 1px solid ${colors.primary};

&.active, &:hover {
    background: ${colors.primary} !important;
    colors: white;
}
`;

const Badge = styled(BootstrapBadge)`
background: ${colors.secondary};
color: white;
`;

const DropdownToggle = styled(Dropdown.Toggle)`
&.dropdown-toggle {
    border: 1px solid ${colors.primary};
    background: none;
    color: ${colors.primary};
}

.show > &, &:hover {
    background: ${colors.primary} !important;
}
`;

export { PrimaryButton, SecondaryButton, OutlinedButton, Badge, DropdownToggle };
