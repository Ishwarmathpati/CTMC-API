<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Extension</name>
   <tag></tag>
   <elementGuidId>d2e2beb2-4317-4a40-91f6-d55e2f5da98e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;destination\&quot; : {\n    \&quot;type\&quot; : \&quot;HTTP\&quot;,\n    \&quot;url\&quot; : \&quot;https://example.azurewebsites.net/api/extension\&quot;,\n    \&quot;authentication\&quot; : {\n      \&quot;type\&quot; : \&quot;AzureFunctions\&quot;,\n      \&quot;key\&quot; : \&quot;some-azure-function-code\&quot;\n    }\n  },\n  \&quot;triggers\&quot; : [ {\n    \&quot;resourceTypeId\&quot; : \&quot;cart\&quot;,\n    \&quot;actions\&quot; : [ \&quot;Create\&quot;, \&quot;Update\&quot; ]\n  } ],\n  \&quot;key\&quot; : \&quot;my-extension\&quot;\n}&quot;,
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
      <webElementGuid>091d600d-443c-4af1-a069-a294245a52cd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b2685e6a-7935-400a-a9b5-ffab8ef997c6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/extensions</restUrl>
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
      <id>b52bb0bd-fd9b-47a3-925d-703b6081517e</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>3e1b8b6f-ae72-446b-a6c5-eb7d6eaa05cd</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
