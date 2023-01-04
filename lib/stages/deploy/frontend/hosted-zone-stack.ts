import * as cdk from 'aws-cdk-lib'
import * as certificatemanager from 'aws-cdk-lib/aws-certificatemanager'
import * as route53 from 'aws-cdk-lib/aws-route53'
import * as constructs from 'constructs'

interface HostedZoneStackProps extends cdk.StackProps {
    region: string,
    domainName: string,
    subdomains: string[],
}

export class HostedZoneStack extends cdk.Stack {
  certificate: certificatemanager.ICertificate
  hostedZone: route53.IHostedZone

  constructor (scope: constructs.Construct, id: string, props: HostedZoneStackProps) {
    super(scope, id, props)

    // Get The Hosted Zone
    this.hostedZone = route53.HostedZone.fromLookup(this, 'HostedZone', {
      domainName: props.domainName
    })

    const subjectAlternativeNames: string[] = []
    props.subdomains.forEach(subdomain => {
      subjectAlternativeNames.push('www.' + props.domainName)
    //   subjectAlternativeNames.push(subdomain.toLowerCase() + '.' + props.domainName)
    //   subjectAlternativeNames.push('www.' + subdomain.toLowerCase() + '.' + props.domainName)
    })

    // Create Certificate
    this.certificate = new certificatemanager.DnsValidatedCertificate(this, 'Certificate', {
      domainName: props.domainName,
      subjectAlternativeNames,
      hostedZone: this.hostedZone,
      region: props.region // standard for acm certs
    })
  }
}
