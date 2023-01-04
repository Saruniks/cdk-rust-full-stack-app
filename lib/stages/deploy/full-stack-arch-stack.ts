import * as cdk from 'aws-cdk-lib'
import * as s3 from 'aws-cdk-lib/aws-s3'
import * as constructs from 'constructs'

import { EbStack } from './backend/eb-stack'
import { RdsStack } from './backend/rds-stack'
import { CloudFrontStack } from './frontend/cloudfront-stack'

export type FullStackAppConfig = {
  auth0Domain: string,
  auth0ClientId: string,
  auth0Authority: string,
  serverPort: string,
}

interface FullStackArchProps extends cdk.StackProps {
  fullStackAppConfig: FullStackAppConfig,
  tag: string,
}

export class FullStackArchStack extends cdk.Stack {
  ebStack: EbStack
  bucket: s3.Bucket
  credentialsParameterName: string

  constructor (scope: constructs.Construct, id: string, props: FullStackArchProps) {
    super(scope, id, props)

    // const hostedZoneStack = new HostedZoneStack(this, 'HostedZoneStack', {
    //   env: {
    //     account: '988317291885',
    //     region: 'us-east-1'
    //   },
    //   region: 'us-east-1',
    //   domainName: 'purecult.link',
    //   subdomains: ['stag', 'prod']
    // })

    const rdsStack = new RdsStack(this, 'RdsStack', {
      env: {
        account: '988317291885',
        region: 'us-east-1'
      },
      yourIpAddres: '213.197.138.142/32',
      securityGroupId: 'sg-054c67583c31aa9e3',
      tag: props.tag
    })

    this.ebStack = new EbStack(this, 'EbStack', {
      fullStackAppConfig: props.fullStackAppConfig,
      credentialsParameterName: rdsStack.credentialsParameterName,
      env: {
        account: '988317291885',
        region: 'us-east-1'
      },
      tag: props.tag
      // certificate: hostedZoneStack.certificate
    })

    const cloudFrontStack = new CloudFrontStack(this, 'CloudFrontStack', {
      env: {
        account: '988317291885',
        region: 'us-east-1'
      },
      domainName: 'purecult.link',
      // certificate: hostedZoneStack.certificate,
      loadBalancerDnsName: this.ebStack.loadBalancerDnsName,
      // hostedZone: hostedZoneStack.hostedZone,
      tag: props.tag
    })

    this.bucket = cloudFrontStack.bucket
    this.credentialsParameterName = rdsStack.credentialsParameterName
  }
}
