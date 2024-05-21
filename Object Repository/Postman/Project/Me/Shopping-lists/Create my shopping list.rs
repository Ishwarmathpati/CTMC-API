<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create my shopping list</name>
   <tag></tag>
   <elementGuidId>e0cee5bd-dca5-4247-b500-739058e8263c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot; : {\n    \&quot;en\&quot; : \&quot;My shopping list\&quot;\n  },\n  \&quot;description\&quot; : {\n    \&quot;en\&quot; : \&quot;Description of my shopping list\&quot;\n  },\n  \&quot;lineItems\&quot; : [ {\n    \&quot;sku\&quot; : \&quot;product-variant-sku\&quot;,\n    \&quot;quantity\&quot; : 5\n  }, {\n    \&quot;productId\&quot; : \&quot;0e131f46-8d1a-4761-9c83-b45ab5d3501e\&quot;,\n    \&quot;variantId\&quot; : 2\n  } ],\n  \&quot;textLineItems\&quot; : [ {\n    \&quot;name\&quot; : {\n      \&quot;en\&quot; : \&quot;My shopping list item\&quot;\n    },\n    \&quot;description\&quot; : {\n      \&quot;en\&quot; : \&quot;This is a good gift idea\&quot;\n    },\n    \&quot;quantity\&quot; : 5\n  } ],\n  \&quot;deleteDaysAfterLastModification\&quot; : 100\n}&quot;,
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
      <webElementGuid>933ba7c9-2db0-4c46-b49f-d6b3e37eccd1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d20722a9-178c-452b-96b8-3bff0211e456</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/me/shopping-lists</restUrl>
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
      <id>b80db16e-daf8-43e9-a207-89a403cedd29</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>5deca015-b813-49c9-96ec-1d4eb826af73</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
