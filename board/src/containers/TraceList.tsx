import React from 'react';
import { 
  ITrace 
} from '~/core/models'; 
import styled from 'styled-components';
import * as styles from '~/styles';
import {Map} from 'immutable';
import {IProps as IChildProps} from '~/connectors/TraceListItem';

const Layout = styled.div`
  display: flex;
<<<<<<< HEAD
  flex-direction: column;
`
||||||| merged common ancestors
`
=======
  flex-direction: column;
  ${styles.card}
`;

const Header = styled.div`
  ${styles.text}
  font-size: 1.5em;
`;
>>>>>>> af488d78fdc3d1a897a54f2bf407e058c4be9d23

interface IProps {
  traceMap: Map<string, ITrace>;
  Child: React.FC<IChildProps>;
}
export default class TraceList extends React.Component<IProps> {
  render = () => {
    const {traceMap, Child} = this.props;
    return (
      <Layout className='card'>
        <Header>
          Traces
        </Header>
        {
          traceMap.toList().map(x => (
            <Child
              key={x.id}
              trace={x}
            />
          ))
        }
      </Layout>
    );
  }
}

