<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Check Message exists by id</name>
   <tag></tag>
   <elementGuidId>e2d9a003-ff44-49b4-aced-0f0027e88e24</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>dac3648d-36d0-41b0-bb9b-cfd23bf36654</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5a7d26d3-061f-4d33-b59e-29404a25433b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>HEAD</restRequestMethod>
   <restUrl>${host}/${project-key}/messages/${message-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>5d91f0d9-9ebd-44d5-9564-ee41b2f88df4</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>d11e0b78-3091-4f97-be28-616852ab7f1b</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.message-id</defaultValue>
      <description></description>
      <id>3fe28545-1979-4eac-a70b-a497f42ac525</id>
      <masked>false</masked>
      <name>message-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
