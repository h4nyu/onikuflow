import React from 'react';
import TraceList from '~/connectors/TraceList';
import styled from 'styled-components';

const Layout = styled.div`
  display: grid;
  grid-template-areas:
    "header header"
    "col0 col1";
  grid-template-columns: auto 1fr;
  grid-template-rows: auto 1fr;
  height: 100vh;
`;
const TraceArea = styled.div`
  grid-area: header;
  height: 52px;
`;

export default class TracePage extends React.Component<{}> {
  render = () => {
    return (
      <Layout>
        <TraceArea>
          <TraceList />
        </TraceArea>
      </Layout>
    );
  }
};
