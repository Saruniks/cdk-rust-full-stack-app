import * as cdk from 'aws-cdk-lib'
import * as certificatemanager from 'aws-cdk-lib/aws-certificatemanager'
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront'
import * as route53 from 'aws-cdk-lib/aws-route53'
import * as route53targets from 'aws-cdk-lib/aws-route53-targets'
import * as s3 from 'aws-cdk-lib/aws-s3'
import * as constructs from 'constructs'

interface CloudFrontProps extends cdk.StackProps {
    domainName: string,
    loadBalancerDnsName: string,
    certificate?: certificatemanager.ICertificate
    hostedZone?: route53.IHostedZone
    tag: string,
}

export class CloudFrontStack extends cdk.Stack {
  cfDistribution: cloudfront.IDistribution
  bucket: s3.Bucket

  constructor (scope: constructs.Construct, id: string, props: CloudFrontProps) {
    super(scope, id, props)

    // S3
    this.bucket = new s3.Bucket(this, 'CloudFrontS3Bucket' + props.tag, {
      publicReadAccess: true,
      websiteIndexDocument: 'index.html',
      websiteErrorDocument: 'index.html'
    })

    // // Deployment
    // new s3Deployment.BucketDeployment(this, 'FullStackDeploymentBucket', {
    //   sources: [s3Deployment.Source.asset(`${__dirname}/../../../../app/crates/ui-app/dist`)],
    //   destinationBucket: this.bucket
    // })

    // Add a cloudfront Function to a Distribution
    // Host header is missing now?
    // const spaFunction = new cloudfront.Function(this, 'SpaFunction' + props.tag, {
    //   code: cloudfront.FunctionCode.fromFile({ filePath: '/home/test/repos/rust-full-stack-cdk/arch/lib/full-stack-arch-stack/spa-function.js' })
    // })

    // const spaFunctionAssociation: cloudfront.FunctionAssociation = {
    //   eventType: cloudfront.FunctionEventType.VIEWER_REQUEST,
    //   function: spaFunction
    // }

    // Handle 404 and 403 error pages to support deep links (rather than using hash-based routing)
    const custom403ErrorResponseProperty: cloudfront.CfnDistribution.CustomErrorResponseProperty = {
      errorCode: 403,

      // the properties below are optional
      errorCachingMinTtl: 0,
      responseCode: 403,
      responsePagePath: '/index.html'
    }

    const custom404ErrorResponseProperty: cloudfront.CfnDistribution.CustomErrorResponseProperty = {
      errorCode: 404,

      // the properties below are optional
      errorCachingMinTtl: 0,
      responseCode: 404,
      responsePagePath: '/index.html'
    }

    let viewerCertificate

    if (props.certificate) {
      viewerCertificate = cloudfront.ViewerCertificate.fromAcmCertificate(
        props.certificate,
        {
          aliases: [
            props.domainName,
            'www.' + props.domainName
          ],
          securityPolicy: cloudfront.SecurityPolicyProtocol.TLS_V1_2_2021 // default
        }
      )
    }

    // CF
    this.cfDistribution = new cloudfront.CloudFrontWebDistribution(this, 'FullStackAppCfDistribution' + props.tag, {
      viewerCertificate,
      originConfigs: [
        {
          s3OriginSource: {
            s3BucketSource: this.bucket
          },
          behaviors: [
            {
              isDefaultBehavior: true
              // functionAssociations: [
              // spaFunctionAssociation
              // ]
            }
          ]
        },
        {
          customOriginSource: {
            domainName: props.loadBalancerDnsName,
            originProtocolPolicy: cloudfront.OriginProtocolPolicy.HTTP_ONLY
          },
          behaviors: [
            {
              isDefaultBehavior: false,
              pathPattern: '/api/*',
              allowedMethods: cloudfront.CloudFrontAllowedMethods.ALL,
              forwardedValues: {
                queryString: true,
                cookies: {
                  forward: 'all'
                },
                headers: ['*']
              }
              // functionAssociations: [
              //   spaFunctionAssociation
              // ]
            }
          ]
        }
      ],
      errorConfigurations: [
        custom403ErrorResponseProperty,
        custom404ErrorResponseProperty
      ]
    })

    if (props.hostedZone) {
      // // Create A Record Custom Domain to CloudFront CDN
      const aRecord = new route53.ARecord(this, 'PurecultRecord', {
        recordName: 'purecult.link',
        target: route53.RecordTarget.fromAlias(new route53targets.CloudFrontTarget(this.cfDistribution)),
        zone: props.hostedZone // try fromHostedZoneAttributes with hostedZoneId & ZoneName
      })

      // // Create www -> non www
      const record = new route53.ARecord(this, 'wwwAliasRecord', {
        recordName: 'www.' + 'purecult.link',
        target: route53.RecordTarget.fromAlias(new route53targets.Route53RecordTarget(aRecord)),
        zone: props.hostedZone
      })
    }
  }
}
