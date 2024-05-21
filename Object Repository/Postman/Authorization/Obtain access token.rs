<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Obtain access token</name>
   <tag></tag>
   <elementGuidId>c5bfe358-8c9b-4361-b9ae-2e147955bd32</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic MW9wUGlHNmwxZURaM2VCbThvWUlmSDRVOkxUT1U4T0dmNHVOVkpIS1RyOVFSR3M2RExRbTRyazJF</value>
      <webElementGuid>45cb195c-f2d4-4449-b5b7-34618c251071</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.auth_url}/oauth/token?grant_type=client_credentials</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.auth_url</defaultValue>
      <description></description>
      <id>b798ef69-0176-466f-a57b-164ecc5edd5b</id>
      <masked>false</masked>
      <name>auth_url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'access_token', &quot;75V5h8KUHN6tVTkjspLWzMGuauOOh-im&quot;)
WS.verifyElementPropertyValue(response, 'access_token', &quot;jjWkzGmXhSblfrm76luV26UPFL61S35A&quot;)
WS.verifyElementPropertyValue(response, 'token_type', &quot;Bearer&quot;)
WS.verifyElementPropertyValue(response, 'expires_in', 172800)
WS.verifyElementPropertyValue(response, 'access_token', &quot;OuInaRmmvSlVv6VCM2oql0cq2tkv96rO&quot;)
WS.verifyElementPropertyValue(response, 'access_token', &quot;NMFr3t-lVLohJbh4Qe3E58dut5x4Hgw4&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
