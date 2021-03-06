import React from 'react';
import TransitionList from '~/connectors/TransitionList';
import PageHeader from '~/connectors/PageHeader';
import Search from '~/connectors/Search';
import styled from 'styled-components';

const Layout = styled.div`
  display: flex;
  flex-direction: column;
  height: 100vh;
`;


export default class TracePage extends React.Component<{}> {
  render = () => {
    return (
      <Layout>
        <PageHeader/>
        <Search/>
        <TransitionList />
      </Layout>
    );
  }
};
