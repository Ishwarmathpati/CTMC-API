<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Type</name>
   <tag></tag>
   <elementGuidId>9aacd998-f9b6-4c1c-a7de-8ee14e39b4ec</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;key\&quot; : \&quot;lineitemtype\&quot;,\n  \&quot;name\&quot; : {\n    \&quot;en\&quot; : \&quot;lineitem\&quot;\n  },\n  \&quot;description\&quot; : {\n    \&quot;en\&quot; : \&quot;description\&quot;\n  },\n  \&quot;resourceTypeIds\&quot; : [ \&quot;line-item\&quot; ],\n  \&quot;fieldDefinitions\&quot; : [ {\n    \&quot;name\&quot; : \&quot;offer_name\&quot;,\n    \&quot;label\&quot; : {\n      \&quot;en\&quot; : \&quot;offer_name\&quot;\n    },\n    \&quot;required\&quot; : false,\n    \&quot;type\&quot; : {\n      \&quot;name\&quot; : \&quot;String\&quot;\n    },\n    \&quot;inputHint\&quot; : \&quot;SingleLine\&quot;\n  } ]\n}&quot;,
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
      <webElementGuid>159a838d-73e9-4cee-bc7e-bbc8918c294b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d258ed72-9f7d-4c0e-ad4a-131830dd28f8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/types</restUrl>
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
      <id>2b5ff14a-42dd-4ae9-8387-db7e36d2ba88</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>b828cd85-8b70-4e92-9e9f-4d742c914e70</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
